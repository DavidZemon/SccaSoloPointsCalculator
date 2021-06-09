import { EOL } from 'os';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import {
  ClassCategoryResults,
  ClassResults,
  EventResults as EventResultsData,
} from '../models';
import { ClassResultsProcessor } from '../services';
import { RamDownload } from './DownloadButton';

interface EventResultsProps extends ComponentPropsWithoutRef<any> {
  results?: EventResultsData;
}

interface EventResultsState {
  csvContent?: string;
}

export class EventResults extends Component<
  EventResultsProps,
  EventResultsState
> {
  constructor(props: Readonly<EventResultsProps>) {
    super(props);
    this.state = {};
  }

  public render() {
    if (this.props.results) {
      return (
        <Row>
          <Col>
            <h2>
              Event Results{' '}
              <Button variant={'secondary'}>
                <FontAwesomeIcon
                  className={'clickable'}
                  icon={faDownload}
                  onClick={() => this.exportCsv()}
                />
              </Button>
            </h2>
            <RamDownload
              filename={'event_results.csv'}
              content={this.state.csvContent}
              contentType={'text/csv'}
              downloadComplete={() => {
                this.setState({ csvContent: undefined });
                console.log('Download is complete');
              }}
            />

            <Accordion>
              {Object.entries(this.props.results).map(
                ([classCategory, categoryResults], index) => (
                  <Card key={index}>
                    <Card.Header>
                      <Accordion.Toggle
                        eventKey={`${index}`}
                        as={Button}
                        variant={'link'}
                      >
                        {classCategory}
                      </Accordion.Toggle>
                    </Card.Header>
                    <Accordion.Collapse eventKey={`${index}`}>
                      <Card.Body>
                        {this.displayCategoryResults(categoryResults)}
                      </Card.Body>
                    </Accordion.Collapse>
                  </Card>
                ),
              )}
            </Accordion>
          </Col>
        </Row>
      );
    } else {
      return null;
    }
  }

  private displayCategoryResults(
    categoryResults: ClassCategoryResults,
  ): JSX.Element[] {
    return EventResults.convertCategoryResults(
      categoryResults,
      (classResults, index) => (
        <Table key={index} striped hover borderless>
          <thead>
            <tr>
              <th colSpan={10}>
                {classResults.carClass} (Trophies: {classResults.trophyCount})
              </th>
            </tr>
            <tr>
              {ClassResultsProcessor.HEADER.slice(0, 6).map((header, index) => (
                <th key={index}>{header}</th>
              ))}
              <th colSpan={12}>Lap Times</th>
              <th>Fastest</th>
              <th>Difference</th>
            </tr>
          </thead>
          <tbody>{this.displayClassResults(classResults)}</tbody>
        </Table>
      ),
    );
  }

  private displayClassResults(classResults: ClassResults): JSX.Element[] {
    const bestLapInClass = classResults.getBestInClass();
    return classResults.drivers.map((driver, index) => {
      const driverBestLap = driver.bestLap();
      return (
        <tr key={index}>
          <td>{driver.trophy ? 'T' : ''}</td>
          <td>{driver.rookie ? 'R' : ''}</td>
          <td>{driver.position}</td>
          <td>{driver.carNumber}</td>
          <td>{driver.name}</td>
          <td>{driver.carDescription}</td>
          {driver.times.map((lapTime, index) => (
            <td key={index}>{lapTime.toString()}</td>
          ))}
          {new Array(12 - driver.times.length).fill(null).map((_, index) => (
            <td key={index} />
          ))}
          <td>{driverBestLap.toString()}</td>
          <td>{driver.difference(bestLapInClass)}</td>
        </tr>
      );
    });
  }

  private exportCsv() {
    const lines = Object.entries(this.props.results!)
      .map(([classCategory, categoryResults]) => [
        [classCategory],
        ...EventResults.exportCategoryResultsToCsv(categoryResults),
      ])
      .flat();
    this.setState({
      csvContent: lines.map((line) => `"${line.join('","')}"`).join(EOL),
    });
  }

  private static exportCategoryResultsToCsv(
    categoryResults: ClassCategoryResults,
  ): string[][] {
    return this.convertCategoryResults(categoryResults, (classResults) => {
      const bestLapInClass = classResults.getBestInClass();
      return [
        [
          classResults.carClass,
          `Drivers: ${classResults.drivers.length}`,
          `Trophies: ${classResults.trophyCount}`,
        ],
        ['TR', 'RK', 'Pos', 'Nbr', "Driver's name, Town", 'Car, Sponsor'],
        ...classResults.drivers.map((driver) => {
          const driverBestLap = driver.bestLap();
          return [
            driver.trophy ? 'T' : '',
            driver.rookie ? 'R' : '',
            `${driver.position}`,
            `${driver.carNumber}`,
            driver.name,
            driver.carDescription,
            ...driver.times.map((t) => t.toString()),
            ...new Array(12 - driver.times.length).fill(''),
            driverBestLap.toString(),
            driver.difference(bestLapInClass),
          ];
        }),
      ];
    }).flat();
  }

  private static convertCategoryResults<T>(
    categoryResults: ClassCategoryResults,
    converter: (classResults: ClassResults, index: number) => T,
  ): T[] {
    return Object.values(categoryResults)
      .filter((classResults) => classResults.drivers.length)
      .sort((a, b) => {
        if (a.carClass < b.carClass) return -1;
        if (a.carClass > b.carClass) return 1;
        else return 0;
      })
      .map(converter);
  }
}
