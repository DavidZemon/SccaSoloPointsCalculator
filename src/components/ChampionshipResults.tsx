import { Component, PropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import {
  ChampionshipResults as ChampionshipResultsData,
  ChampionshipType,
  IndexedChampionshipResults,
} from '../models';
import { PaxService } from '../services';
import { RamDownload } from './DownloadButton';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';

interface ChampionshipResultsProps extends PropsWithoutRef<any> {
  paxService: PaxService;
  results?: ChampionshipResultsData;
}

interface ChampionshipResultsState {
  downloadName?: string;
  downloadData?: BlobPart;
}

export class ChampionshipResults extends Component<
  ChampionshipResultsProps,
  ChampionshipResultsState
> {
  constructor(props: Readonly<ChampionshipResultsProps>) {
    super(props);
    this.state = {};
  }

  render() {
    if (
      this.props.results &&
      Object.values(this.props.results).some((v) => !!v)
    ) {
      return [
        <Row key={0}>
          <Col>
            <h2>Championship Results</h2>

            <Accordion>
              {Object.entries(this.props.results)
                // Class results need to be displayed separately
                .filter(([championshipType, _]) => championshipType !== 'Class')
                // Get some nice type hints going
                .map(([key, value]) => {
                  return [key, value] as [
                    ChampionshipType,
                    IndexedChampionshipResults,
                  ];
                })
                .map(([championshipType, results]) => (
                  <Card key={championshipType}>
                    <Card.Header key={championshipType}>
                      <Accordion.Toggle
                        eventKey={championshipType}
                        as={Button}
                        variant={'link'}
                      >
                        {results.year} {championshipType} Championship Standings
                      </Accordion.Toggle>
                      <Button variant={'secondary'} disabled>
                        <FontAwesomeIcon
                          className={'clickable'}
                          icon={faDownload}
                          onClick={() => {
                            /* TODO */
                          }}
                        />
                        &nbsp;&lt;&mdash; This button doesn't do anything yet
                      </Button>
                    </Card.Header>

                    <Accordion.Collapse eventKey={championshipType}>
                      <Card.Body>
                        <Table striped hover borderless>
                          <thead>
                            <tr>
                              <th>Rank</th>
                              <th>Driver</th>
                              {new Array(results.results[0].points.length)
                                .fill(null)
                                .map((_, index) => (
                                  <th key={index}>Event #{index + 1}</th>
                                ))}
                              <th>Total Points</th>
                              <th>
                                Best{' '}
                                {ChampionshipResults.calculateEventsToCount(
                                  results.results[0].points,
                                )}{' '}
                                of {results.results[0].points.length}
                              </th>
                            </tr>
                          </thead>

                          <tbody>
                            {results.results
                              .sort((d1, d2) => d1.position - d2.position)
                              .map((driver, index) => (
                                <tr key={index}>
                                  <td>{driver.position}</td>
                                  <td>{driver.name}</td>
                                  {driver.points.map((points, index) => (
                                    <td key={index}>{points}</td>
                                  ))}

                                  <td>
                                    {driver.points.reduce(
                                      (sum, p) => sum + p,
                                      0,
                                    )}
                                  </td>

                                  <td>
                                    {ChampionshipResults.calculateChampionshipPoints(
                                      driver.points,
                                    )}
                                  </td>
                                </tr>
                              ))}
                          </tbody>
                        </Table>
                      </Card.Body>
                    </Accordion.Collapse>
                  </Card>
                ))}
            </Accordion>
          </Col>
        </Row>,
        <RamDownload
          key={1}
          filename={this.state.downloadName}
          content={this.state.downloadData}
          contentType={'application/vnd.ms-excel'}
          downloadComplete={() => {
            this.setState({ downloadData: undefined });
            console.log('Download is complete');
          }}
        />,
      ];
    } else {
      return null;
    }
  }

  private static calculateClassPoints(fastest: number, actual: number): number {
    if (fastest === actual) {
      return 10000;
    } else {
      return Math.round((fastest / actual) * 10_000);
    }
  }

  private static calculateChampionshipPoints(
    points: (number | undefined)[],
  ): number {
    const eventCount = points.length;
    const fleshedOutPoints = points.map((p) => p || 0);
    if (eventCount < 4) {
      return fleshedOutPoints.reduce((sum, p) => sum + p, 0);
    } else {
      const eventsToCount = this.calculateEventsToCount(points);
      return fleshedOutPoints
        .sort()
        .reverse()
        .slice(0, eventsToCount)
        .reduce((sum, p) => sum + p, 0);
    }
  }

  private static calculateEventsToCount(
    points: (number | undefined)[],
  ): number {
    if (points.length < 4) return points.length;
    else return Math.round(points.length / 2) + 2;
  }
}
