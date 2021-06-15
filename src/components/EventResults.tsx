import { EOL } from 'os';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import {
  ClassCategoryResults,
  ClassResults,
  Driver,
  EventResults as EventResultsData,
} from '../models';
import { EventResultsParser, PaxService } from '../services';
import { RamDownload } from './DownloadButton';

interface EventResultsProps extends ComponentPropsWithoutRef<any> {
  paxService: PaxService;
  results?: EventResultsData;
}

interface EventResultsState {
  csvContent?: string;
  exportFilename?: string;
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
      return [
        <Row key={0} className={'top-buffer'}>
          <Col>
            <h2>Event Results</h2>

            <Accordion>
              <Card>
                <Card.Header key={'class'}>
                  <Accordion.Toggle
                    eventKey={'class'}
                    as={Button}
                    variant={'link'}
                  >
                    Results by Class
                  </Accordion.Toggle>
                  <Button variant={'secondary'}>
                    <FontAwesomeIcon
                      className={'clickable'}
                      icon={faDownload}
                      onClick={() => this.exportResultsByClassCsv()}
                    />
                  </Button>
                </Card.Header>

                <Accordion.Collapse eventKey={'class'}>
                  <Card.Body>
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
                  </Card.Body>
                </Accordion.Collapse>
              </Card>

              {this.displayCombinedResults(true)}

              {this.displayCombinedResults(false)}
            </Accordion>
          </Col>
        </Row>,
        <RamDownload
          key={1}
          filename={this.state.exportFilename}
          content={this.state.csvContent}
          contentType={'text/csv'}
          downloadComplete={() =>
            this.setState({ csvContent: undefined, exportFilename: undefined })
          }
        />,
      ];
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
              {EventResultsParser.HEADER.slice(0, 6).map((header, index) => (
                <th key={index}>{header}</th>
              ))}
              <th>Region</th>
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
          <td>{driver.region}</td>
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

  private displayCombinedResults(pax: boolean): JSX.Element {
    const key = pax ? 'pax' : 'raw';

    const drivers = Object.values(this.props.results!)
      .map((categoryResults) => Object.values(categoryResults))
      .flat()
      .map((classResults) => classResults.drivers)
      .flat();
    const sortedDrivers = drivers
      .map(
        (driver) =>
          [
            driver,
            (driver.bestLap().time || Infinity) *
              (pax
                ? this.props.paxService.getMultiplierFromLongName(
                    driver.carClass,
                  )
                : 1),
          ] as [Driver, number],
      )
      .sort(([_1, d1Time], [_2, d2Time]) => d1Time - d2Time);
    const fastestOfDay = sortedDrivers[0][1];

    return (
      <Card>
        <Card.Header key={key}>
          <Accordion.Toggle eventKey={key} as={Button} variant={'link'}>
            {pax ? 'PAX' : 'Raw'} Results
          </Accordion.Toggle>
          <Button
            variant={'secondary'}
            onClick={() =>
              this.exportCombinedResultsToCsv(sortedDrivers, fastestOfDay, pax)
            }
          >
            <FontAwesomeIcon className={'clickable'} icon={faDownload} />
          </Button>
        </Card.Header>

        <Accordion.Collapse eventKey={key}>
          <Card.Body>
            <Table striped hover borderless>
              <thead>
                <tr>
                  <th>Position</th>
                  <th>Class</th>
                  <th>Car #</th>
                  <th>Name</th>
                  <th>Car</th>
                  <th>Region</th>
                  <th>{pax ? 'PAX' : 'Raw Corrected'} Time</th>
                  <th>Difference</th>
                </tr>
              </thead>

              <tbody>
                {sortedDrivers.map(([driver, driverBestTime], index) => (
                  <tr key={index}>
                    <td>{index + 1}</td>
                    <td>{driver.carClass}</td>
                    <td>{driver.carNumber}</td>
                    <td>{driver.name}</td>
                    <td>{driver.carDescription}</td>
                    <td>{driver.region}</td>
                    <td>
                      {driver
                        .bestLap()
                        .toString(
                          pax
                            ? this.props.paxService.getMultiplierFromLongName(
                                driver.carClass,
                              )
                            : undefined,
                        )}
                    </td>
                    <td>
                      {driver.difference(
                        fastestOfDay,
                        pax
                          ? this.props.paxService.getMultiplierFromLongName(
                              driver.carClass,
                            )
                          : undefined,
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </Table>
          </Card.Body>
        </Accordion.Collapse>
      </Card>
    );
  }

  private exportResultsByClassCsv() {
    const lines = Object.entries(this.props.results!)
      .map(([classCategory, categoryResults]) => [
        [classCategory],
        ...this.exportCategoryResultsToCsv(categoryResults),
      ])
      .flat();
    this.setState({
      exportFilename: 'event_results.csv',
      csvContent: lines.map((line) => `"${line.join('","')}"`).join(EOL),
    });
  }

  private exportCategoryResultsToCsv(
    categoryResults: ClassCategoryResults,
  ): string[][] {
    return EventResults.convertCategoryResults(
      categoryResults,
      (classResults) => {
        const shortCarClass = classResults.carClass
          .split(' ')
          .map((word) => word[0])
          .join('');
        const paxMultiplier = this.props.paxService.getMultiplierFromLongName(
          classResults.carClass,
        );
        const bestLapInClass = classResults.getBestInClass();
        const bestIndexTime = (bestLapInClass || Infinity) * paxMultiplier;
        return [
          [`${shortCarClass} - ${classResults.carClass}`],
          ...classResults.drivers.map((driver, index) => {
            const driverBestLap = driver.bestLap();
            return [
              `${driver.position}`,
              driver.name,
              driver.carDescription,
              shortCarClass,
              `${driver.carNumber}`,
              driverBestLap.toString(undefined, false),
              driverBestLap.toString(paxMultiplier, false),
              index === 0
                ? ''
                : driver.difference(
                    (classResults.drivers[index - 1].bestLap().time ||
                      Infinity) * paxMultiplier,
                    paxMultiplier,
                  ),
              driver.difference(bestIndexTime, paxMultiplier),
              driver.region,
            ];
          }),
        ];
      },
    ).flat();
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

  private exportCombinedResultsToCsv(
    sortedDrivers: [Driver, number][],
    fastestOfDay: number,
    pax: boolean,
  ): void {
    const results = [
      [
        'Position',
        'Class',
        'Car #',
        'Name',
        'Car',
        `${pax ? 'PAX' : 'Raw'} Time`,
        'Difference',
      ],
      ...sortedDrivers.map(([driver, time], index) => [
        `${index + 1}`,
        driver.carClass,
        `${driver.carNumber}`,
        driver.name,
        driver.carDescription,
        driver
          .bestLap()
          .toString(
            pax
              ? this.props.paxService.getMultiplierFromLongName(driver.carClass)
              : undefined,
          ),
        driver.difference(
          fastestOfDay,
          pax
            ? this.props.paxService.getMultiplierFromLongName(driver.carClass)
            : undefined,
        ),
      ]),
    ];
    this.setState({
      exportFilename: `event_${pax ? 'pax' : 'raw'}_results.csv`,
      csvContent: results.map((row) => `"${row.join('","')}"`).join(EOL),
    });
  }
}
